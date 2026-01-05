<script setup>
import { BREAKPOINTS } from '@/constants/breakpoints';

const props = defineProps({
  src: {
    type: String,
    required: true,
  },
  srcset: {
    type: String,
    default: undefined,
  },
  media: {
    type: Object,
    default: undefined,
    validator(value) {
      const validKeys = Object.keys(BREAKPOINTS);

      return Object.keys(value).every((key) => validKeys.includes(key));
    },
  },
  webp: {
    type: Object,
    default: undefined,
    validator(value) {
      const validKeys = ['srcset', ...Object.keys(BREAKPOINTS)];

      return Object.keys(value).every((key) => validKeys.includes(key));
    },
  },
  alt: {
    type: String,
    default: '',
  },
  loading: {
    type: String,
    default: 'lazy',
    validator: (value) => ['lazy', 'eager'].includes(value),
  },
  fetchPriority: {
    type: String,
    default: undefined,
    validator: (value) => ['high', 'low', 'auto'].includes(value),
  },
  objectFit: {
    type: String,
    default: undefined,
    validator: (value) => ['cover', 'contain', 'fill'].includes(value),
  },
});

const {
  app: { baseURL },
} = useRuntimeConfig();

const mimeTypes = {
  webp: 'image/webp',
};

const sources = computed(() => {
  if (!props.media && !props.webp) return;

  const breakpointsEntriesAsc = Object.entries(BREAKPOINTS).sort(
    (a, b) => a[1] - b[1],
  );

  const sources = breakpointsEntriesAsc.reduce(
    (sources, [breakpointKey, breakpointValue]) => {
      const jpgPngSource = getSource({
        srcset: props.media?.[breakpointKey],
        breakpoint: breakpointValue,
      });
      const webpSource = getSource({
        srcset: props.webp?.[breakpointKey],
        breakpoint: breakpointValue,
        type: mimeTypes.webp,
      });

      if (webpSource) sources.push(webpSource);
      if (jpgPngSource) sources.push(jpgPngSource);

      return sources;
    },
    [],
  );

  const webpSrcsetSource = getSource({
    srcset: props.webp?.srcset,
    type: mimeTypes.webp,
  });

  if (webpSrcsetSource) sources.push(webpSrcsetSource);

  return sources;
});

function getSource({ srcset, breakpoint, type }) {
  if (!srcset) return;

  if (baseURL && baseURL !== '/') {
    srcset = applyBaseUrl(srcset, baseURL);
  }

  let media;

  if (breakpoint) {
    media = `(max-width: ${breakpoint - 0.02}px)`;
  }

  return {
    srcset,
    media,
    type,
  };
}

function applyBaseUrl(srcset = '', baseUrl = '') {
  return srcset
    .split(',')
    .map((url) => combineURLs(baseUrl, url.trim()))
    .join(', ');
}
</script>

<template>
  <picture class="picture">
    <source
      v-for="(
        { srcset: sourceSrcset, media: sourceMedia, type: sourceType }, idx
      ) in sources"
      :key="idx"
      :srcset="sourceSrcset"
      :type="sourceType"
      :media="sourceMedia"
    />

    <CImage
      :src="src"
      :srcset="srcset"
      :alt="alt"
      :loading="loading"
      :fetchpriority="fetchPriority"
      class="picture__image"
      :class="{
        [`picture__image--fit--${objectFit}`]: !!objectFit,
      }"
    />
  </picture>
</template>

<style scoped lang="scss">
.picture {
  &__image {
    width: 100%;
    height: 100%;

    &--fit {
      &--cover {
        object-fit: cover;
      }

      &--contain {
        object-fit: contain;
      }
    }
  }
}
</style>
