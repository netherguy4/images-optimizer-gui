<script setup>
const { $gsap: gsap } = useNuxtApp();

const props = defineProps({
  tag: {
    type: String,
    default: 'ul',
  },
  duration: {
    type: Number,
    default: 0.25,
  },
});

const onBeforeEnter = (el) => {
  gsap.set(el, {
    autoAlpha: 0,
    height: 0,
    marginBottom: 0,
    marginTop: 0,
    paddingTop: 0,
    paddingBottom: 0,
    overflow: 'hidden',
    z: 0,
    backfaceVisibility: 'hidden',
  });
};

const onEnter = (el, done) => {
  gsap.to(el, {
    autoAlpha: 1,
    height: 'auto',
    marginTop: '',
    marginBottom: '',
    paddingTop: '',
    paddingBottom: '',
    onComplete: done,
    duration: props.duration,
    ease: 'power1.out',
  });
};

const onLeave = (el, done) => {
  gsap.set(el, {
    overflow: 'hidden',
    z: 0,
    backfaceVisibility: 'hidden',
  });

  gsap.to(el, {
    autoAlpha: 0,
    height: 0,
    marginTop: 0,
    marginBottom: 0,
    paddingTop: 0,
    paddingBottom: 0,
    onComplete: done,
    duration: props.duration,
    ease: 'power1.in',
  });
};
</script>

<template>
  <TransitionGroup
    :tag="props.tag"
    :css="false"
    class="smooth-list"
    @before-enter="onBeforeEnter"
    @enter="onEnter"
    @leave="onLeave"
  >
    <slot />
  </TransitionGroup>
</template>

<style lang="scss" scoped>
.smooth-list {
  position: relative;
  list-style: none;
  transition: height $time-normal $ease;
}
</style>
