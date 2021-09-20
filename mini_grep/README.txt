-cat
takes N args which is to be filenames (paths),
print their concatination (and \n between)
>>cargo run cat src/text.txt src/text2.txt src/text.txt
flavor flavour 
tic tak toe
rust flawer 
flat
Flat
F

-------------------------------
 	THE NEW TEXT!!
-------------------------------

flavor flavour 
tic tak toe
rust flawer 
flat
Flat
F


-grep
takes 2 args which is to be substring and filename (path),
print all file's strings that conteins substring
>>cargo run grep fla src/text.txt
flavor flavour 
rust flawer 
flat


-find
takes 1 or 2 args which is to be [start_directory] and sougth file,
print all directories that conteins such file
>cargo run find ~ cat.rs 
"/home/mike/Rust/projects/mini_grep/src/cat.rs"
