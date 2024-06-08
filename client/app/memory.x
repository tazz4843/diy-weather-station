MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  BOOT2                             : ORIGIN = 0x10000000, LENGTH = 0x100
  
  BOOTLOADER_STATE                  : ORIGIN = 0x10009000, LENGTH = 8K
  FLASH                             : ORIGIN = 0x1000b000, LENGTH = 876K
  DFU                               : ORIGIN = 0x100e6000, LENGTH = 880K
  WIFI_FIRMWARE					    : ORIGIN = 0x101c2000, LENGTH = 256K

  RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(BOOT2);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(BOOT2);

__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(BOOT2);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(BOOT2);
