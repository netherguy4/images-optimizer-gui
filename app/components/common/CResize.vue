<template>
  <div class="resize">
    <slot />
  </div>
</template>

<script setup></script>

<style lang="scss" scoped>
$design-width: 1600;
$design-height: 967;
$default-size: 16;
$min-width: 320px;
$min-height: 600px;

.resize {
  // font-size: min(
  //  calc($default-size * max($min-width, 100vw) / $design-width),
  //  calc($default-size * max($min-height, 100vh) / $design-height)
  // );

  font-size: $default-size + px;

  // @include media-breakpoint-down(sm) {
  //   font-size: min(calc($default-size * max($min-width, 100vw) / 320));
  // }
}
</style>
