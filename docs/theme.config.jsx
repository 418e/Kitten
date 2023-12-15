export default {
  useNextSeoProps() {
    return {
      titleTemplate: "%s – Kitten",
    };
  },
  head: (
    <>
      <meta name="viewport" content="width=device-width, initial-scale=1.0" />
      <meta property="og:title" content="Kitten HTML template" />
      <meta property="og:description" content="The next site builder" />
      <meta name="keywords" content="kitten, html template, html compiler, kitten compiler, cat, cat compiler, javascript, 418e, tron, tronlang, kittenlang, kitten language, kitten docs, kitten documentation" />
      <link rel="shortcut icon" href="/kitten.png" type="image/x-icon" />
    </>
  ),
  project: {
    link: "https://github.com/418e/kitten",
  },
  docsRepositoryBase: "https://github.com/418e/kitten/tree/main/docs",
  darkMode: true,
  logo: (
    <>
      <img src={"/kitten.png"} height={32} width={32} />
      <span style={{ marginLeft: ".4em", fontWeight: 800 }}>Kitten</span>
    </>
  ),
  sidebar: {
    toggleButton: true,
  },
  themeSwitch: {
    useOptions() {
      return {
        light: "Light",
        dark: "Dark",
        system: "System",
      };
    },
  },
  footer: {
    text: "MIT 2023 © Kitten.",
  },
};
