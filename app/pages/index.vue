<script setup>
definePageMeta({
  key: 'fixed',
});

const filesStore = useFilesStore();
const { items } = storeToRefs(filesStore);
</script>

<template>
  <ASmoothList tag="div" class="pages-index">
    <SectionHead description="sections.head.description" />

    <SectionAddFiles />

    <template v-if="items?.length">
      <SectionFilesList />

      <SectionOptions />
    </template>
  </ASmoothList>
</template>

<style lang="scss" scoped>
.pages-index {
  display: flex;
  flex-direction: column;
  gap: em(32);
  padding-block: em(32);
}
</style>
