# bitmap
<img width="536" height="238" alt="bitmap logo" src="https://github.com/user-attachments/assets/357162d5-5d10-490d-bb42-2ace3c291e4f" />


Hashmap implementation utilizing a bitmap array, attempting to improve cache performance

# the idea 
I wanted to see if i can make a hashmap that had some slight time performances, not to replace but as a learning experience. Rust is a fun language to write stuff from scratch. 

Since I am not smart enough to make my own hashing and I am lazy to do anything other than linear collision, I decided to implement the one thing my college has actually taught: cache performance and how memory works. 

a basic explanation of a cache is, a cache is made out of cache blocks, that hold words (data). so when searching for said data, it has to look block by block, and word by word. We have limited space in each, forgot the amount since it also varies, but the idea is that if we load bytes that can hold multiple entry positions, we have much more spatial locality than just loading normal entries. So in theory it should lead to less misses and writebacks. Well the results will show I am indeed wrong. 

# the current progress
There are two implementations, u8 and u64 bytes. 

