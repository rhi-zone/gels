import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid(
  defineConfig({
    vite: {
      optimizeDeps: {
        include: ['mermaid'],
      },
    },
    title: 'Gels',
    description: 'Trait-based grammar inference engine',

    base: '/gels/',

    themeConfig: {
      nav: [
        { text: 'Guide', link: '/introduction' },
        { text: 'Architecture', link: '/architecture' },
        { text: 'rhi', link: 'https://docs.rhi.zone/' },
      ],

      sidebar: [
        {
          text: 'Guide',
          items: [
            { text: 'Introduction', link: '/introduction' },
            { text: 'Getting Started', link: '/getting-started' },
          ],
        },
        {
          text: 'Internals',
          items: [
            { text: 'Architecture', link: '/architecture' },
            { text: 'Trait Detectors', link: '/trait-detectors' },
          ],
        },
      ],

      socialLinks: [
        { icon: 'github', link: 'https://github.com/rhi-zone/gels' },
      ],

      search: {
        provider: 'local',
      },

      editLink: {
        pattern: 'https://github.com/rhi-zone/gels/edit/master/docs/:path',
        text: 'Edit this page on GitHub',
      },
    },
  }),
)
