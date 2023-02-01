# Bin2C
Converter of binary files to C array

## Usage

~~~
Bin2C [-ascii] file.bin file.c c_array
~~~

Argument description:

| Argument     | Description                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------- |
| **-ascii**   | optional, if provided, encode all ASCII printable characters as ASCII characters in the C array |
| **file.bin** | specifies binary input filename                                                                 |
| **file.c**   | specifies output C filename                                                                     |
| **c_array**  | specifies the name of the C array                                                               |

ASCII argument means, for example, that character **A** will be encoded as **'A'** in the array instead of **0x41**.

## Tests

### Microsoft Windows

If located in the root folder of this repository (where **README.md** file is) the test results in the **Test** folder 
can be produced with the following commands:

#### GitBash / PowerShell

~~~
./Exe/Win/Debug/Bin2C -ascii ./Test/Test.bin ./Test/Test_ASCII.c Test_ASCII_array
~~~

~~~
./Exe/Win/Debug/Bin2C ./Test/Test.bin ./Test/Test.c Test_array
~~~

#### Command Prompt

~~~
.\Exe\Win\Debug\Bin2C -ascii .\Test\Test.bin .\Test\Test_ASCII.c Test_ASCII_array
~~~

~~~
.\Exe\Win\Debug\Bin2C .\Test\Test.bin .\Test\Test.c Test_array
~~~

## Source code compilation

### Prerequisites

Installed **Rust**.

To install **Rust** follow the instructions [here](https://www.rust-lang.org/tools/install).

### Compilation

In the folder **Rust\Src** execute the following commands:

#### Debug

~~~
cargo build
~~~

Executable will be created in the **\Rust\target\Debug** folder.

#### Release

~~~
cargo build --release
~~~

Executable will be created in the **\Rust\target\Release** folder.

\Note Currently, only executable files for Microsoft windows are provided, but you can build 
      executable files for MAC or Linux from source file using Rust on those Platforms.
