import adapter from '@sveltejs/adapter-auto';
import sveltePreprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: sveltePreprocess({
    postcss: true, // This enables PostCSS
  }),

  kit: {
    adapter: adapter()
  }
};

export default config;

