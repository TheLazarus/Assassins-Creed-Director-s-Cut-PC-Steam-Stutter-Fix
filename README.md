# Assassins-Creed-Director-s-Cut-PC-Steam-Stutter-Fix
A small patch I wrote for fixing the stuttering issue in AC Director's Cut PC Steam Version


## Use Executable Directly

- Copy the AssassinsCreed_StutterFix executable to /steamapps/common/Assassins Creed directory and run it
- After running this, you should be able to see two new files named AssassinsCreed_Dx9_patched.exe and AssassinsCreed_Dx10_patched.exe
- Take a backup of your original exe's
- Rename these files back to AssassinsCreed_Dx9.exe and AssassinsCreed_Dx10.exe (replace the original ones after taking backup)
- Run the game using Steam "Play" button, or you can also launch directly via these executables
- Enjoy!!


## Compile Source

- Install rustup
- To compile for older 32-bit machines - add target i686-pc-windows-msvc using "rustup target add i686-pc-windows-msvc"
- Run "cargo build --target i686-pc-windows-msvc --release"
- Enjoy!!