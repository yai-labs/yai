#!/usr/bin/env python3
import json
import os
import socket
import stat
import struct
import time

YAI_FRAME_MAGIC = 0x59414950
YAI_PROTOCOL_IDS_VERSION = 1
YAI_CMD_HANDSHAKE = 0x0102
YAI_CMD_PING = 0x0101
ENV_FMT = "<II36s36sIHBBII"
ACK_FMT = "<IIHBB"
REQ_FMT = "<II32s"
SOCK_PATH = os.environ.get("YAI_SOCK_PATH", os.path.expanduser("~/.yai/run/control.sock"))


def make_envelope(cmd_id, ws_id="system", trace_id="trace-0001", role=2, arming=1, payload=b""):
    ws_bytes = ws_id.encode("utf-8")[:36].ljust(36, b"\0")
    trace_bytes = trace_id.encode("utf-8")[:36].ljust(36, b"\0")
    return struct.pack(
        ENV_FMT,
        YAI_FRAME_MAGIC,
        YAI_PROTOCOL_IDS_VERSION,
        ws_bytes,
        trace_bytes,
        cmd_id,
        role,
        arming,
        0,
        len(payload),
        0,
    )


def read_envelope(sock):
    env_size = struct.calcsize(ENV_FMT)
    env_bytes = b""
    while len(env_bytes) < env_size:
        chunk = sock.recv(env_size - len(env_bytes))
        if not chunk:
            break
        env_bytes += chunk

    if len(env_bytes) < env_size:
        env_bytes = env_bytes.ljust(env_size, b"\0")

    magic, version, ws_bytes, trace_bytes, cmd_id, role, arming, _, payload_len, _ = struct.unpack(
        ENV_FMT, env_bytes
    )
    ws_id = ws_bytes.decode("utf-8").rstrip("\0")
    trace_id = trace_bytes.decode("utf-8").rstrip("\0")

    payload = b""
    remaining = payload_len
    start_time = time.time()
    while remaining > 0:
        chunk = sock.recv(remaining)
        if not chunk:
            if time.time() - start_time > 2:
                break
            time.sleep(0.01)
            continue
        payload += chunk
        remaining -= len(chunk)

    return {
        "magic": magic,
        "version": version,
        "command_id": cmd_id,
        "role": role,
        "arming": arming,
        "ws_id": ws_id,
        "trace_id": trace_id,
        "payload": payload,
    }


def print_response(resp, title):
    print(f"\n=== {title} ===")
    print(f"magic=0x{resp['magic']:08X} version={resp['version']} cmd_id=0x{resp['command_id']:X}")
    print(f"ws_id={resp['ws_id']} trace_id={resp['trace_id']}")
    if resp["payload"]:
        try:
            print("payload:", json.dumps(json.loads(resp["payload"]), indent=2))
        except Exception:
            print("payload (raw bytes):", resp["payload"])


def assert_valid_frame(resp, expected_cmd):
    if resp["magic"] != YAI_FRAME_MAGIC:
        raise RuntimeError(f"bad magic: {resp['magic']}")
    if resp["version"] != YAI_PROTOCOL_IDS_VERSION:
        raise RuntimeError(f"bad version: {resp['version']}")
    if resp["command_id"] != expected_cmd:
        raise RuntimeError(f"unexpected command id: {resp['command_id']}")


def send_handshake(sock):
    payload = struct.pack(REQ_FMT, YAI_PROTOCOL_IDS_VERSION, 0, b"yai-test")
    sock.sendall(make_envelope(YAI_CMD_HANDSHAKE, payload=payload) + payload)
    resp = read_envelope(sock)
    assert_valid_frame(resp, YAI_CMD_HANDSHAKE)
    if len(resp["payload"]) != struct.calcsize(ACK_FMT):
        raise RuntimeError("invalid handshake ack size")
    server_version, _, _, status, _ = struct.unpack(ACK_FMT, resp["payload"])
    if server_version != YAI_PROTOCOL_IDS_VERSION or status != 2:
        raise RuntimeError("invalid handshake ack content")
    print_response(resp, "HANDSHAKE RESPONSE")


def send_ping(sock):
    sock.sendall(make_envelope(YAI_CMD_PING))
    resp = read_envelope(sock)
    assert_valid_frame(resp, YAI_CMD_PING)
    if not resp["payload"]:
        raise RuntimeError("empty ping payload")
    body = json.loads(resp["payload"].decode("utf-8"))
    if body.get("status") != "pong":
        raise RuntimeError(f"unexpected ping body: {body}")
    print_response(resp, "PING RESPONSE")


def main():
    if not os.path.exists(SOCK_PATH):
        print(f"SKIP: ingress socket not found: {SOCK_PATH}")
        print("Set YAI_SOCK_PATH or start the yai runtime before running this test.")
        return 0

    mode = os.stat(SOCK_PATH).st_mode
    if not stat.S_ISSOCK(mode):
        print(f"SKIP: path exists but is not a Unix socket: {SOCK_PATH}")
        return 0

    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    try:
        s.connect(SOCK_PATH)
    except FileNotFoundError:
        print(f"SKIP: ingress socket not found: {SOCK_PATH}")
        return 0
    except ConnectionRefusedError:
        print(f"SKIP: ingress socket exists but no server is accepting connections: {SOCK_PATH}")
        return 0
    except OSError as exc:
        print(f"SKIP: cannot connect to socket {SOCK_PATH}: {exc}")
        return 0

    print(f"[+] Connected to {SOCK_PATH}")
    try:
        send_handshake(s)
        for i in range(3):
            print(f"\n[+] Ping attempt {i + 1}")
            send_ping(s)
            time.sleep(0.2)
    finally:
        s.close()
        print("\n[+] Connection closed")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
