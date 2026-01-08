<script setup>
import { formatSize } from '@/utils/helpers';
import Simplebar from 'simplebar-vue';
import 'simplebar/dist/simplebar.min.css';

const filesStore = useFilesStore();
const { items, totalItems, totalSize } = storeToRefs(filesStore);
</script>

<template>
  <div class="files-list-block">
    <div class="container">
      <div class="files-list-block__container">
        <div class="files-list-block__header">
          <div class="files-list-block__header-part">
            <div class="files-list-block__title">
              <p class="h2-r">
                {{ $t('sections.files-list.title') }} ({{ totalItems }})
              </p>
            </div>

            <div class="files-list-block__subtitle">
              <p class="s1-r">
                {{ $t('sections.files-list.subtitle') }}
                {{ formatSize(totalSize) }}
              </p>
            </div>
          </div>

          <div class="files-list-block__header-part">
            <UiButton
              size="sm"
              theme="warn"
              icon="trash"
              :title="$t('common.clear-all')"
              @click="filesStore.clearAll()"
            />
          </div>
        </div>

        <Simplebar class="files-list-block__scroll">
          <ASmoothList tag="div" class="files-list-block__content">
            <CardListItem
              v-for="item in items"
              :key="item.id"
              :item="item"
              @remove="filesStore.removeById"
            />
          </ASmoothList>
        </Simplebar>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.files-list-block {
  &__container {
    display: flex;
    flex-direction: column;
    gap: em(24);
    padding: em(24);
    background-color: $background-color-primary;
    border: 1px solid $border-color-secondary;
    border-radius: em(16);
    transition: $time-normal $ease;
    transition-property: background-color, border-color;
  }

  &__header {
    display: flex;
    flex-shrink: 0;
    align-items: flex-start;
    justify-content: space-between;
  }

  &__header-part {
    display: flex;
    flex-direction: column;
    gap: em(4);
  }

  &__subtitle {
    color: $text-color-secondary;
    transition: color $time-normal $ease;
  }

  &__scroll {
    max-height: em(300);
    padding-right: em(12);
    margin-block: em(-12);
    margin-right: em(-12);

    :deep(.simplebar-content-wrapper) {
      mask-image: linear-gradient(
        to bottom,
        transparent,
        black em(12),
        black calc(100% - em(12)),
        transparent
      );
    }

    :deep(.simplebar-scrollbar::before) {
      z-index: 1;
      background-color: $text-color-primary;
      opacity: 0.5;
      transition: all $time-normal $ease;
    }

    :deep(.simplebar-scrollbar) {
      &:hover::before {
        opacity: 0.8;
      }
    }

    :deep(.simplebar-track.simplebar-vertical) {
      width: 6px;
      background-color: transparent;
    }
  }

  &__content {
    display: flex;
    flex-direction: column;
    gap: em(12);
    padding-block: em(12);
  }
}
</style>
