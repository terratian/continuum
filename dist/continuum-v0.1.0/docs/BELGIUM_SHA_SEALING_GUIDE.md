# CONTINUUM: BELGIUM (BOIP) & SHA-256 OPEN-TIMESTAMPS APPLICATION GUIDE
**Inventor:** Joshua Sultan (Burlington, North Carolina, US)  
**Legal Framework:** Benelux Convention on Intellectual Property (BCIP Art. 4.4bis) & Bitcoin OpenTimestamps Protocol  

## 1. Purpose of the Belgian BOIP i-DEPOT
The Benelux Office for Intellectual Property (BOIP) i-DEPOT vault establishes an official, legally binding date-certainty certificate across all 27 European Union member states for unpublished trade secrets and trade specifications without public disclosure.

## 2. Standard Operating Procedure
1. Run `./scripts/seal_ip_ledger.sh` to generate the SHA-256 manifest and Bitcoin timestamp proof (`.ots`).
2. Upload `.ip_vault/continuum_ip_seal_*.tar.gz` to `boip.int` under a Strictly Confidential i-DEPOT envelope.
3. Log the resulting BOIP certificate number in `docs/IP_LEDGER.log`.
