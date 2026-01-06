<script setup>
import { formatSize } from '@/utils/helpers';
import Simplebar from 'simplebar-vue';
import 'simplebar/dist/simplebar.min.css';

const filesStore = useFilesStore();
const { items, totalItems, totalSize } = storeToRefs(filesStore);
</script>

<template>
  <AFade>
    <div v-if="items?.length" class="files-list-block">
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
                title="common.clear-all"
                @click="filesStore.clearAll()"
              />
            </div>
          </div>

          <Simplebar class="files-list-block__scroll">
            <div v-auto-animate class="files-list-block__content">
              <CardListItem
                v-for="item in items"
                :key="item.id"
                :item="item"
                @remove="filesStore.removeById"
              />
            </div>
          </Simplebar>
        </div>
      </div>
    </div>
  </AFade>
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
    margin-right: em(-12);

    :deep(.simplebar-scrollbar::before) {
      background-color: $background-color-additional;
      opacity: 1;

      &:hover {
        opacity: 1;
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
  }
}
</style>
