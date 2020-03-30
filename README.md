# Rust Json

## Why
I've made some web apps, and I needed to parse large (>20M) json files.  
On Android `JSON.parse` does the job pretty well, but on iOS, well, it just crashes with JSON files over 500kb.  
I've resorted to using a plain javascript parser (`LosslessJSON`) wich works pretty well.  
I've forked and edited this library so that it would use normal numbers (instead of LosslessNumbers)
as I didn't need that feature which is slower and uses more memory (my fork is called `JsonLib`).  

Now, I've exported a JSON parser from the Rust library `serde_json` and compiled it to wams.

## Performance
It's faster (30-50% faster) than `LosslessJSON`, but slower (8 times slower) than native `JSON.parse`.  
I think most of the time is spent on transferring data between wasm and js, idk, please help.  


## Usage
Just copy `pkg/rs_json_bg.wasm` and `pkg/json_rs.js` in a folder.
Then, do this:
```html
<head>
  <script src="json_rs.js"></script>
  <script>
    wasm_bindgen('json_rs_bg.wasm').then(() => {
      window.RustJson = wasm_bindgen
    })
  </script>
</head>
```

## Notes
There are actually two parse functions: `parse` and `_parse`, the first one uses serde_json, the second one uses `serde_json_wasm` which should be better for constrained environments, but it does not work. Maybe one day it will.
