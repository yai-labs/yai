"""
ICE Runtime — Event Validator
=============================

RFC-ICE-003 · Event Model
RFC-ICE-007 · Event Taxonomy

Questo modulo decide se un evento ICE:
- può esistere
- è strutturalmente valido
- è causalmente valido
- è temporalmente valido
- è autorizzato

Se un evento fallisce qui:
→ NON ESISTE
→ NON entra nel sistema
"""

from __future__ import annotations

from datetime import datetime
from typing import Optional, Set

from .event import ICEEvent, EventID
from .taxonomy import is_valid_event_type
from .authority import is_origin_authorized


# ============================================================================
# Errori Canonici (Fondativi)
# ============================================================================

class EventValidationError(Exception):
    """Errore base di validazione evento ICE."""


class StructuralViolation(EventValidationError):
    """Violazione strutturale dell'evento."""


class TaxonomyViolation(EventValidationError):
    """Evento non definito dalla tassonomia RFC-ICE-007."""


class AuthorityViolation(EventValidationError):
    """Origine non autorizzata a emettere l'evento."""


class TemporalViolation(EventValidationError):
    """Violazione della monotonicità temporale."""


class CausalityViolation(EventValidationError):
    """Violazione della catena causale."""


# ============================================================================
# Validator Sovrano (Stateless)
# ============================================================================

class EventValidator:
    """
    Validator stateless del Kernel Eventi.

    NON:
    - modifica eventi
    - corregge errori
    - applica fallback

    Valida o rifiuta.
    """

    # ------------------------------------------------------------------
    # Entry Point Unico
    # ------------------------------------------------------------------

    @staticmethod
    def validate(
        event: ICEEvent,
        *,
        known_event_ids: Set[EventID],
        last_timestamp: Optional[datetime] = None,
    ) -> None:
        """
        Valida un evento ICE.

        Args:
            event: evento da validare
            known_event_ids: EventID già noti nel Run
            last_timestamp: timestamp dell'ultimo evento valido

        Raises:
            EventValidationError
        """
        EventValidator._validate_structure(event)
        EventValidator._validate_taxonomy(event)
        EventValidator._validate_authority(event)
        EventValidator._validate_temporal(event, last_timestamp)
        EventValidator._validate_causality(event, known_event_ids)

    # ------------------------------------------------------------------
    # Validazioni Canoniche
    # ------------------------------------------------------------------

    @staticmethod
    def _validate_structure(event: ICEEvent) -> None:
        """
        Valida la struttura minimale dell'evento.

        NOTA:
        Molti invarianti sono già enforce in ICEEvent.__post_init__.
        Qui si verifica solo coerenza d'uso.
        """
        if not isinstance(event.event_id, str):
            raise StructuralViolation("event_id must be str")

        if not isinstance(event.run_id, str):
            raise StructuralViolation("run_id must be str")

        if not isinstance(event.timestamp, datetime):
            raise StructuralViolation("timestamp must be datetime")

    @staticmethod
    def _validate_taxonomy(event: ICEEvent) -> None:
        """
        Verifica che l'evento esista nella tassonomia chiusa.
        """
        if not is_valid_event_type(event.event_type):
            raise TaxonomyViolation(
                f"Unknown event_type '{event.event_type}'"
            )

    @staticmethod
    def _validate_authority(event: ICEEvent) -> None:
        """
        Verifica che l'origine sia autorizzata a emettere l'evento.
        """
        if not is_origin_authorized(
            origin=event.origin,
            event_type=event.event_type,
        ):
            raise AuthorityViolation(
                f"Origin '{event.origin}' not authorized for "
                f"event_type '{event.event_type}'"
            )

    @staticmethod
    def _validate_temporal(
        event: ICEEvent,
        last_timestamp: Optional[datetime],
    ) -> None:
        """
        Garantisce monotonicità temporale per Run.
        """
        if last_timestamp is None:
            return

        if event.timestamp < last_timestamp:
            raise TemporalViolation(
                "Event timestamp goes backwards in time"
            )

    @staticmethod
    def _validate_causality(
        event: ICEEvent,
        known_event_ids: Set[EventID],
    ) -> None:
        """
        Valida la catena causale dell'evento.

        Regole:
        - causality è None o Tuple[EventID, ...]
        - ogni EventID referenziato DEVE esistere
        """
        if event.causality is None:
            return

        if not isinstance(event.causality, tuple):
            raise CausalityViolation("causality must be tuple")

        for parent_id in event.causality:
            if not isinstance(parent_id, str):
                raise CausalityViolation(
                    "causality entries must be EventID (str)"
                )

            if parent_id not in known_event_ids:
                raise CausalityViolation(
                    f"Unknown causal EventID '{parent_id}'"
                )


# ============================================================================
# Clausola Finale
# ============================================================================

"""
Questo modulo NON decide se un evento è utile.
NON decide se è intelligente.
NON decide se è desiderabile.

Decide solo una cosa:
→ l'evento può esistere nel Runtime ICE?

Se NO:
→ l'evento viene rigettato
→ il Run deve abortire
"""
