import { defineConfig } from "vitepress";

export default defineConfig({
  title: "PhenoData",
  description: "Phenotype data-layer workspace for SurrealDB, Postgres/pgvector, and query planning.",
  base: process.env.GITHUB_PAGES === "true" ? "/phenoData/" : "/",
  cleanUrls: true,
  themeConfig: {
    logo: { text: "PhenoData" },
    nav: [
      { text: "Guide", link: "/guide/" },
      { text: "Crates", link: "/crates/" },
      { text: "GitHub", link: "https://github.com/KooshaPari/phenoData" },
    ],
    sidebar: [
      {
        text: "PhenoData",
        items: [
          { text: "Overview", link: "/" },
          { text: "Guide", link: "/guide/" },
          { text: "Crates", link: "/crates/" },
          { text: "Operations", link: "/operations/" },
        ],
      },
    ],
    socialLinks: [{ icon: "github", link: "https://github.com/KooshaPari/phenoData" }],
    search: {
      provider: "local",
    },
  },
});
