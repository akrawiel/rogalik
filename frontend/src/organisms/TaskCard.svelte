<script lang="ts">
  import clsx from 'clsx';
  import { nanoid } from 'nanoid';

  import RoundButton from '@/atoms/RoundButton.svelte';
  import trackingCanvasState, {
    onCardClose,
    onCardOpen,
  } from '@/store/trackingCanvas';
  import type { OpenCardData } from '@/store/trackingCanvas';
  import { createEventDispatcher } from 'svelte';

  export let containerClass: string;
  export let openOnClick = false;

  let isOpen = false;
  let cardElement: HTMLDivElement | undefined;
  let cardContainerElement: HTMLDivElement | undefined;
  let cardData: OpenCardData | undefined;
  let dummyCardElement: HTMLDivElement | undefined;

  const dispatch = createEventDispatcher();

  function toggleIsOpen() {
    if (!cardElement) return;

    isOpen = !isOpen;

    if (isOpen && cardData) {
      onCardOpen(cardData);
    } else {
      onCardClose();
    }
  }

  function onRightClick() {
    if (!isOpen) {
      toggleIsOpen();
    }
  }

  function onClick() {
    if (isOpen || openOnClick) {
      toggleIsOpen();
    } else {
      dispatch('start');
    }
  }

  $: if (cardElement && cardContainerElement && dummyCardElement) {
    const boundingClientRect = cardElement.getBoundingClientRect();

    const dummyCardWidth =
      parseInt(getComputedStyle(dummyCardElement).width, 10) || 0;
    const dummyCardHeight =
      parseInt(getComputedStyle(dummyCardElement).height, 10) || 0;

    cardData = {
      cardId: nanoid(),
      top: boundingClientRect.top + dummyCardHeight / 2,
      left: boundingClientRect.left + dummyCardWidth / 2,
    };
  }

  $: if (
    $trackingCanvasState?.cardId &&
    $trackingCanvasState?.cardId !== cardData?.cardId
  ) {
    isOpen = false;
  }
</script>

<div
  bind:this={cardContainerElement}
  class={clsx(containerClass, 'card-container', isOpen ? 'open' : '')}
>
  <div bind:this={cardElement} class={clsx('card', isOpen ? 'open' : '')}>
    <div class={clsx('button-container', isOpen ? 'open' : '')}>
      <RoundButton on:click={onClick} on:contextmenu={onRightClick} size="lg">
        <slot name="icon" {isOpen} />
      </RoundButton>
    </div>
    <div class="card-header">
      <div class="flex items-center p-2 h-full">
        <slot name="header" />
      </div>
    </div>
    <slot />
  </div>
</div>

<div class="card open dummy" bind:this={dummyCardElement} />

<style lang="scss">
  .card-container {
    z-index: 40;

    &.open,
    &:hover {
      z-index: 50;
    }
  }

  .card:hover:not(.open) {
    width: 16rem;

    @screen sm {
      width: 20rem;
    }
  }

  .card {
    @apply bg-hex-ffffff absolute shadow-none;
    border-radius: 2.25rem;
    height: 4.5rem;
    left: -2.25rem;
    overflow: hidden;
    padding-top: 4.5rem;
    transition: all 0.25s ease;
    top: -2.25rem;
    width: 4.5rem;

    &.open {
      @apply shadow-md;
      height: 32rem;
      width: 16rem;

      @screen sm {
        width: 20rem;
      }
    }

    &.dummy {
      @apply hidden shadow-none;
    }

    @screen <sm {
      width: 16rem;
    }
  }

  .card-header {
    @apply absolute top-0 left-0 rounded-full bg-primary-dark shadow-md overflow-hidden;
    height: 4.5rem;
    padding-left: 4.5rem;
    white-space: nowrap;
    width: 100%;
    z-index: 1;
  }

  .button-container {
    @apply absolute top-0 left-0;
    z-index: 2;
  }
</style>
