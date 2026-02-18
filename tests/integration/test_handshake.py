#!/usr/bin/env python3
import socket
import struct
import json
import time
import os

# ====================================================
# Costanti del protocollo (allineate al kernel)
# ====================================================
YAI_FRAME_MAGIC = 0x59414950
YAI_PROTOCOL_IDS_VERSION = 1
YAI_CMD_HANDSHAKE = 0x0102
YAI_CMD_PING      = 0x0101
SOCK_PATH = os.environ.get("YAI_SOCK_PATH", os.path.expanduser("~/.yai/run/root/control.sock"))

# ====================================================
# Envelope builder
# ====================================================
def make_envelope(cmd_id, ws_id="root", trace_id="trace-0001", role=0, arming=1, payload=b""):
    ws_bytes = ws_id.encode("utf-8")[:36].ljust(36, b"\0")
    trace_bytes = trace_id.encode("utf-8")[:36].ljust(36, b"\0")
    payload_len = len(payload)
    return struct.pack(
        "<II36s36sHBBIII",
        YAI_FRAME_MAGIC,
        YAI_PROTOCOL_IDS_VERSION,
        ws_bytes,
        trace_bytes,
        cmd_id,
        role,
        arming,
        0,            # pad
        payload_len,
        0             # checksum
    )

# ====================================================
# Envelope reader robusto
# ====================================================
def read_envelope(sock):
    env_bytes = b""
    while len(env_bytes) < 96:
        chunk = sock.recv(96 - len(env_bytes))
        if not chunk:
            break
        env_bytes += chunk

    if len(env_bytes) < 96:
        # Padding automatico
        env_bytes = env_bytes.ljust(96, b"\0")

    unpacked = struct.unpack("<II36s36sHBBIII", env_bytes)
    magic, version, ws_bytes, trace_bytes, cmd_id, role, arming, pad, payload_len, checksum = unpacked
    ws_id = ws_bytes.decode("utf-8").rstrip("\0")
    trace_id = trace_bytes.decode("utf-8").rstrip("\0")

    # Leggi payload se presente
    payload = b""
    remaining = payload_len
    start_time = time.time()
    while remaining > 0:
        try:
            chunk = sock.recv(remaining)
        except ConnectionResetError:
            break
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
        "payload": payload
    }

# ====================================================
# Print envelope
# ====================================================
def print_response(resp, title):
    print(f"\n=== {title} ===")
    print(f"magic=0x{resp['magic']:08X} version={resp['version']} cmd_id=0x{resp['command_id']:X}")
    print(f"ws_id={resp['ws_id']} trace_id={resp['trace_id']}")
    if resp['payload']:
        try:
            print("payload:", json.dumps(json.loads(resp['payload']), indent=4))
        except Exception:
            print("payload (raw bytes):", resp['payload'])

# ====================================================
# Multi-shot handshake + ping
# ====================================================
def send_handshake(sock):
    sock.sendall(make_envelope(YAI_CMD_HANDSHAKE))
    resp = read_envelope(sock)
    print_response(resp, "HANDSHAKE RESPONSE")
    return resp

def send_ping(sock):
    sock.sendall(make_envelope(YAI_CMD_PING))
    resp = read_envelope(sock)
    print_response(resp, "PING RESPONSE")
    return resp

# ====================================================
# Main
# ====================================================
def main():
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.connect(SOCK_PATH)
    print(f"[+] Connected to {SOCK_PATH}")

    try:
        # --- Handshake
        send_handshake(s)

        # --- Ping multipli
        for i in range(3):
            print(f"\n[+] Ping attempt {i+1}")
            send_ping(s)
            time.sleep(0.2)

        # --- Possibili test futuri (attach workspace, query, etc.)
        # esempio:
        # s.sendall(make_envelope(YAI_CMD_ATTACH, ws_id="system"))
        # resp_attach = read_envelope(s)
        # print_response(resp_attach, "ATTACH RESPONSE")

    except BrokenPipeError:
        print("[!] Server ha chiuso la connessione")
    finally:
        s.close()
        print("\n[+] Connection closed")

if __name__ == "__main__":
    main()
