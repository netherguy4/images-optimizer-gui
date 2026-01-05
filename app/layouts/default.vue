<template>
  <div class="layouts-default">
    <!-- <LHeader ref="headerRef" class="layouts-default__header" /> -->

    <main class="layouts-default__content">
      <div class="layouts-default__wrapper">
        <slot class-name="layouts-default__view" />
      </div>
    </main>

    <!-- <LFooter /> -->

    <div id="modal-root" />

    <ClientOnly>
      <ModalsContainer />
    </ClientOnly>
  </div>
</template>

<script setup>
import { ModalsContainer } from 'vue-final-modal';
import { useScrollLock } from '@/composables/useScrollLock';

const scrollLock = useScrollLock();

defineProps({
  className: {
    type: String,
    default: undefined,
  },
});

onBeforeUnmount(() => {
  scrollLock.unlock();
});
</script>

<style scoped lang="scss">
.layouts-default {
  $parent: &;

  display: flex;
  flex-direction: column;

  &:deep(#{$parent}__view) {
    display: flex;
    flex-grow: 1;
    flex-direction: column;
  }

  &__header {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 999;
    width: 100%;
  }

  &__wrapper {
    display: flex;
    flex-direction: column;
    min-height: 100dvh;
  }
}
</style>
