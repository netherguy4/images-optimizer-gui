<script setup>
const props = defineProps({
  src: {
    type: String,
    required: true,
  },
  srcset: {
    type: String,
    default: undefined,
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
});

const {
  app: { baseURL },
} = useRuntimeConfig();

const imageSrc = computed(() => {
  return combineURLs(baseURL, props.src);
});

const srcSet = computed(() => {
  if (!props.srcset) return;

  return props.srcset
    .split(',')
    .map((src) => combineURLs(baseURL, src.trim()))
    .join(', ');
});
</script>

<template>
  <img
    :src="imageSrc"
    :srcset="srcSet"
    :alt="alt"
    :loading="loading"
    :fetchpriority="fetchPriority"
  />
</template>
