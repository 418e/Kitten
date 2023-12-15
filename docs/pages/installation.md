# Installation

## Linux/MacOS

To download and install Kitten on **Linux**, you need to open your terminal and enter the following command:

```bash
curl -o kitten https://tronlang.org/kitten
sudo mv kitten /usr/local/bin/
sudo chmod +x /usr/local/bin/kitten
```

## Windows

Soon

## Using Kitten

To start building **Kitten** applications first create `index.kitten` file (or whatever you want, as long as the suffix is `.kitten`) 

```kitten
div[]{
    "Hello, World!"
}
```
Open the terminal and run:

```bash
kitten run index
```

Enjoy codding :)
