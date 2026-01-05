<script setup>
defineProps({
  title: {
    type: String,
    default: '',
  },
  icon: {
    type: String,
    default: '',
  },
  iconPos: {
    type: String,
    default: 'left',
    validator: (pos) => ['left', 'right'].includes(pos),
  },
});
</script>

<template>
  <button class="ui-button">
    <CIcon
      v-if="icon && iconPos === 'left'"
      class="ui-button__icon"
      :name="icon"
    />

    <slot>
      <span v-if="!$slots.default && title" class="ui-button__font">
        {{ title }}
      </span>
    </slot>

    <CIcon
      v-if="icon && iconPos === 'right'"
      class="ui-button__icon"
      :name="icon"
    />
  </button>
</template>

<style lang="scss" scoped>
.ui-button {
  display: flex;
  gap: em(5);

  &__font {
    @include i1;
  }

  &__icon {
    width: em(24);
    height: em(24);
    object-fit: contain;
  }
}
</style>
