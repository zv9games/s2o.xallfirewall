# S2O firewall stack plan — durable copy

**Saved:** 2026-07-19  
**Also at:**
- `C:\ZV9\S2O_FIREWALL_STACK_PLAN.md`
- `C:\ZV9\s2o.s2o_net_lib\S2O_FIREWALL_STACK_PLAN.md`
- `C:\ZV9\s2o.xallfirewall\S2O_FIREWALL_STACK_PLAN.md`
- `C:\ZV9\zv9.SSXL\docs\S2O_FIREWALL_STACK_PLAN.md`

**Context:** ~$300/yr license-or-signing cost shelved project; no portfolio revenue to justify renewing.

---
# S2O firewall stack strategy â€” shelved for cost (saved)

**Saved:** 2026-07-19  
**Durable copies:** `C:\ZV9\S2O_FIREWALL_STACK_PLAN.md` (and under s2o repos when written)

---

## Why the project was shelved (your constraint)

You hit a **recurring cost ~$300/year** (commercial license and/or ongoing fee to stay relevantâ€”often this is **Npc commercial**, **code-signing / EV**, or **vendor support**, not â€œWinDivert is closed-sourceâ€).

With **no current return from the S2O / ZV9 repo array**, that annual burn is **not justified**. Correct product decision: **shelve deep packet/divert work** until revenue or a zero-recurring-cost path is enough.

This plan assumes: **$0/year third-party stack fees** unless something is already paid for.

---

## Three jobs (do not conflate)

| Job | Meaning | Needs paid divert/signing stack? |
|-----|---------|----------------------------------|
| **A. Policy firewall** | Rules via Windows Firewall / WFP user APIs | **No** |
| **B. Sniff/capture** | See packets | Often free tier limits or Npcap commercial for redistribution |
| **C. Divert/modify** | Intercept/drop/reinject in stack | Driver + usually **signing** and/or commercial redistributable terms |

**Shelved:** B/C as product differentiators.  
**Keep alive (cheap):** A â€” what `s2o_net_lib` CLI already aims at.

A **userspace DLL alone** cannot own kernel divert; policy DLL **can** ship without annual filter-vendor tax.

---

## Cost-aware options

### Tier 0 â€” $0 recurring (recommended while shelved / bootstrapping)

**Product = policy controller only**

- Windows Firewall COM / documented Firewall + WFP **user-mode** APIs  
- Defender hooks only where free/documented  
- Ship: **CLI + optional `s2o_net_lib.dll` (cdylib)** + later free UI  
- **No** WinDivert redistributable product story, **no** custom `.sys`, **no** $300/yr license  

Fits: â€œI have no return yet.â€  
Misses: fancy live packet dashboard / MITM-style control (xallfirewall dream).

### Tier 1 â€” One-time or already sunk cost only

- If you **already paid** a year: use it until expiry; **do not renew** without revenue  
- Use period for: API design, UI, policy featuresâ€”not expanding paid surface  

### Tier 2 â€” When revenue exists (revisit)

| Need | Typical cost driver |
|------|---------------------|
| Redistribute capture/divert | Vendor commercial license (e.g. Npcap-class deals) |
| Install your own or third-party **.sys** | **Code signing** (often hundreds/year) + Partner Center |
| Custom **WFP callout** | Engineering time + signing, not a monthly â€œhackâ€ |

Only reopen Tier 2 with **paying users or a sponsored goal**.

### Explicitly not recommended

- Unsigned kernel hacks to avoid fees (broken by modern Windows, liability)  
- Pretending a free DLL replaces a signed divert driver  
- Paying $300/yr for a portfolio that isnâ€™t monetized  

---

## WinDivert note (clarify free vs paid)

- WinDivert core is **LGPL** (open source). Pain is often **signing/install**, support, or **other** stack pieces (Npc commercial, certs).  
- You already have WinDivert bits under `s2o.xallfirewall/lib/` â€” fine for **local dev**, careful for **shipping** (LGPL + driver trust).  
- Annual $300 may have been **signing cert**, **Npc OEM**, or similarâ€”not necessarily WinDivert source access. Record what the invoice was for when you revisit.

---

## Repo roles while shelved

| Repo | Role while $0 |
|------|----------------|
| **s2o.s2o_net_lib** | **Canonical** â€” policy CLI/backend; optional cdylib later |
| **s2o.xallfirewall** | **Frozen front-end + WinDivert vendor** â€” no paid path until justified |
| Path/API mismatch between them | Document only; fix when un-shelving |

---

## Un-shelve checklist (future)

1. Revenue or sponsor covers **â‰¥** annual license/signing  
2. Choose: **policy-only product** (stay Tier 0) vs **divert product** (budget signing + vendor)  
3. Fix `xallfirewall` â†’ `../s2o.s2o_net_lib` path and API  
4. Do **not** start custom kernel until Tier 2 budget is real  

---

## DLL story under $0 constraint

```text
NOW / CHEAP:
  s2o_net_lib.dll  = policy API only (Firewall/WFP user mode)
  CLI               = same crate

LATER / PAID:
  + divert.dll + signed .sys or licensed WinDivert/Npc redistributable
```

---

## One-sentence fridge

> **Until S2O pays for itself, ship only free Windows policy APIs (CLI/DLL); leave packet-divert and $300/yr stack fees on the shelf with xallfirewall.**

---

## Related local paths

- `C:\ZV9\s2o.s2o_net_lib`  
- `C:\ZV9\s2o.xallfirewall`  
- `C:\ZV9\S2O_NET_LIB_ASSESSMENT.md`  
- `C:\ZV9\S2O_XALLFIREWALL_ASSESSMENT.md`  
- This plan: `C:\ZV9\S2O_FIREWALL_STACK_PLAN.md`
