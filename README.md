# `product_index`

A tool for get the ND indicies of a grid from a 1D index.


## Usage

```bash
arr1=($(seq 1 1 5))
arr2=($(seq 1 1 6))

N=12

# prints 2 0
product_index $N "${#arr1[@]}" "${#arr2[@]}" 

# stores i1=2 i2=0
read -r i1 i2 <<< $(product_index $N "${#arr1[@]}" "${#arr2[@]}")

# Prints 3,1
echo "x,y = ${arr1[$i1]}, ${arr2[$i2]}"
```


## Installation

Not currently on crates.io. To clone the repo and use `cargo install --path .`

### Step by step
1. If cargo is not installed, install the rust compilers:
    ``` shell
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    (See https://www.rust-lang.org/tools/install for more options)
2.  Clone the repo:
    ```shell
    git clone https://github.com/chappertron/product_index.git
    ```
3. `cd` into the repo folder
4. Run the cargo install command:
    ```shell
    cargo install --path . 
    ```
You should now be able to access the `product_index` binary from the command line.
