/* CC1312R7: 704 KB flash, 144 KB SRAM (Cortex-M4F @ 48 MHz default RCOSC).
 *
 * NOTE: the last flash page holds the CCFG (Customer Configuration). A valid
 * CCFG is required for the device to boot standalone and to keep the debug
 * interface enabled. It is not emitted here yet; add a `.ccfg` section before
 * running on real hardware. Under a debugger the core is held and started
 * directly, so RTT/semihosting examples still execute without it.
 */
MEMORY
{
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 704K
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 144K
}
