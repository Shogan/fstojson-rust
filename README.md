# fstojson-rust

A fairly simple CLI tool to traverse a target directory & output the collected 
hierarchy to JSON. This is my first attempt at writing something in the Rust language.

My first attempt at this in Rust failed when trying to create nested trees containing nodes for the FS objects. (I was plagued with memory moves owned issues
when trying to reference `Vec` lists containing Structs with child `Vec` lists recursively). Uses the **indextree** crate to help with tree structures.

## Usage

Output a target path in the filesystem (as well as it's contents) as a JSON string:

`fstojson <PATH>`

Output a target path in the filesystem (recursive) and all contents as a JSON string:

`fstojson <PATH> -r`

Add the `-p` argument to output the JSON in prettified print.

`fstojson <PATH> -r -p`

```json
{
  "name": "temp/f1/f1sub1",
  "relative_path": "temp/f1/f1sub1",
  "absolute_path": "/Users/seand/Git/sean/fstojson/temp/f1/f1sub1",
  "node_type": "Directory",
  "children": [
    {
      "name": "a-file.txt",
      "relative_path": "temp/f1/f1sub1/a-file.txt",
      "absolute_path": "/Users/seand/Git/sean/fstojson/temp/f1/f1sub1/a-file.txt",
      "node_type": "File"
    },
    {
      "name": "empty-dir",
      "relative_path": "temp/f1/f1sub1/empty-dir",
      "absolute_path": "/Users/seand/Git/sean/fstojson/temp/f1/f1sub1/empty-dir",
      "node_type": "Directory"
    }
  ]
}
```

### fstojson with jq

Of course you can pass the output of fstojson to tools like jq via the pipeline:

Getting just the absolute paths of files within a particular directory:

`fstojson temp/somedir | jq '.children[].absolute_path`

```text
/Users/seand/Git/sean/fstojson/temp/f1/f1sub1/a-file.txt
/Users/seand/Git/sean/fstojson/temp/f1/f1sub1/empty-dir
```

## Notes

- Directory content at each level may not necessarily be sorted alphabetically.
