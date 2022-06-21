[Writing an OS in Rust](https://os.phil-opp.com/)

**Caller-saved** registers (AKA **volatile** registers or **call-clobbered**) are used to hold temporary quantities that need not be preserved across calls.

**Callee-saved registers** (AKA **non-volatile** registers, or **call-preserved**) are used to hold long-lived values that should be preserved across calls.

**TODO**
1. Quesion
2. Lab Bonus section
3. the question which set "#![no_main]" in x86-64-linux baremetal. 
4. add "sleep 5 seconds" in start entry. 
