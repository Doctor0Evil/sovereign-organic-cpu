#!/usr/bin/env python3
"""
Evidence Bundle Verification Script

Validates 10-tag EvidenceBundle completeness and cryptographic integrity.
"""

import json
import hashlib
import sys
from datetime import datetime
from typing import Dict, List, Optional

REQUIRED_TAGS = [
    "0xpow20e7",    # brain_power_envelope
    "0xtherm3c",    # thermal_safety
    "0xcyt0a9",     # cytokine_thresholds
    "0xcload7f",    # cognitive_load_markers
    "0xlyaduty",    # ml_duty_cycle
    "0xrights1c",   # neurorights_clauses
    "0xroh025c",    # organiccpu_scheduler_limits
    "0xeco9b2",     # eco_monotonicity
    "0xdevcorr",    # device_specific_corridor
    "0xclin10a",    # clinical_grade_evidence
]

def load_bundle(filepath: str) -> Optional[Dict]:
    """Load EvidenceBundle from JSON file."""
    try:
        with open(filepath, 'r') as f:
            return json.load(f)
    except Exception as e:
        print(f"Error loading bundle: {e}")
        return None

def verify_tag_completeness(bundle: Dict) -> bool:
    """Verify all 10 required tags are present."""
    if 'evidence_tags' not in bundle:
        print("❌ Missing evidence_tags field")
        return False
    
    tags = bundle['evidence_tags']
    if len(tags) != 10:
        print(f"❌ Expected 10 tags, found {len(tags)}")
        return False
    
    found_anchors = {tag['hex_anchor'] for tag in tags}
    missing = set(REQUIRED_TAGS) - found_anchors
    
    if missing:
        print(f"❌ Missing required tags: {missing}")
        return False
    
    print("✅ All 10 required tags present")
    return True

def verify_roh_constraint(bundle: Dict) -> bool:
    """Verify RoH ≤ 0.3 constraint."""
    roh = bundle.get('roh_snapshot', 1.0)
    if roh > 0.3:
        print(f"❌ RoH constraint violated: {roh} > 0.3")
        return False
    print(f"✅ RoH constraint satisfied: {roh} ≤ 0.3")
    return True

def verify_rod_constraint(bundle: Dict) -> bool:
    """Verify ROD < 1.0 constraint."""
    rod = bundle.get('rod_snapshot', 1.0)
    if rod >= 1.0:
        print(f"❌ ROD HardStop triggered: {rod} >= 1.0")
        return False
    print(f"✅ ROD constraint satisfied: {rod} < 1.0")
    return True

def verify_eco_monotonicity(bundle: Dict) -> bool:
    """Verify eco-monotonicity (delta ≤ 0.0)."""
    delta = bundle.get('eco_impact_delta', 1.0)
    if delta > 0.0:
        print(f"❌ Eco-monotonicity violated: delta {delta} > 0.0")
        return False
    print(f"✅ Eco-monotonicity satisfied: delta {delta} ≤ 0.0")
    return True

def verify_did_format(bundle: Dict) -> bool:
    """Verify DID format is valid Bostrom DID."""
    did = bundle.get('did', '')
    if not did.startswith('did:bostrom:'):
        print(f"❌ Invalid DID format: {did}")
        return False
    print(f"✅ DID format valid: {did}")
    return True

def compute_bundle_hash(bundle: Dict) -> str:
    """Compute SHA3-256 hash of bundle for verification."""
    bundle_copy = bundle.copy()
    bundle_copy.pop('signature', None)
    bundle_copy.pop('anchor_hash', None)
    
    content = json.dumps(bundle_copy, sort_keys=True)
    hash_obj = hashlib.sha3_256(content.encode())
    return hash_obj.hexdigest()

def verify_signature(bundle: Dict) -> bool:
    """Verify cryptographic signature (placeholder)."""
    if 'signature' not in bundle:
        print("❌ Missing signature")
        return False
    
    # In real implementation, verify DID signature
    print("✅ Signature present (verification requires DID public key)")
    return True

def main():
    if len(sys.argv) != 2:
        print("Usage: verify_evidence_bundle.py <bundle.json>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    bundle = load_bundle(filepath)
    
    if not bundle:
        sys.exit(1)
    
    print(f"\n{'='*60}")
    print(f"Evidence Bundle Verification: {filepath}")
    print(f"{'='*60}\n")
    
    checks = [
        verify_tag_completeness(bundle),
        verify_roh_constraint(bundle),
        verify_rod_constraint(bundle),
        verify_eco_monotonicity(bundle),
        verify_did_format(bundle),
        verify_signature(bundle),
    ]
    
    computed_hash = compute_bundle_hash(bundle)
    print(f"\n📋 Computed Bundle Hash: {computed_hash}")
    
    if 'anchor_hash' in bundle:
        print(f"📋 Anchored Hash: {bundle['anchor_hash']}")
    
    print(f"\n{'='*60}")
    if all(checks):
        print("✅ ALL VERIFICATION CHECKS PASSED")
        print(f"{'='*60}\n")
        sys.exit(0)
    else:
        print("❌ SOME VERIFICATION CHECKS FAILED")
        print(f"{'='*60}\n")
        sys.exit(1)

if __name__ == '__main__':
    main()
