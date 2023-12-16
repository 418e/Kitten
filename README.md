<div align="center">

![Logo](./kitten.png)

# Kitten

#### HTML template compiler

Created to get read of closing tags, simplify syntax and replace so many javascript libraries/frameworks

</div>

## Installation

```bash
curl -o kitten https://kitten.tronlang.org/v/latest
sudo mv kitten /usr/local/bin/
sudo chmod +x /usr/local/bin/kitten
```

## Usage/Examples

```kitten
import[from:components/nav; element:$1; as:Nav; ]
import[from:components/footer; element:$2; as:Footer; ]

Nav[]
div[classname:header;]{
   p[]{"Welcome to the Kitten!"}
   span[]{
    (){
      return localStorage.getItem("name");
    }
  }
}
Footer[]
```

- `import[from:components/nav; element:$1; as:Nav; ]` - imports element with key:$1 attribute from `components/nav.kitten` as `Nav[]`
- `div[classname:header]{}` same as <div classname="header"></div>
- `(){return localStorage.getItem("name")}` - returns item "name" from localstorage

## Compiling

```bash
kitten run <filename>
```

- `<filename>` - name of the `.kitten` file (`main.kitten`, `index.kitten`, `blog.kitten`)

## Documentation

[kitten.tronlang.org](https://kitten.tronlang.org/)
