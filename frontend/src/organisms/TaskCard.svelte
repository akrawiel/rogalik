<script lang="ts">
  import clsx from 'clsx';
  import { nanoid } from 'nanoid';

  import RoundButton from '@/atoms/RoundButton.svelte';
  import PlayIcon from '@/icons/PlayIcon.svelte';
  import trackingCanvasState, {
    onCardClose,
    onCardOpen,
  } from '@/store/trackingCanvas';
  import type { OpenCardData } from '@/store/trackingCanvas';
  import { onMount } from 'svelte';

  export let containerClass: string;

  let isOpen = false;
  let cardElement: HTMLDivElement | undefined;
  let cardData: OpenCardData | undefined;

  function toggleIsOpen() {
    if (!cardElement) return;

    isOpen = !isOpen;

    if (isOpen && cardData) {
      onCardOpen(cardData);
    } else {
      onCardClose();
    }
  }

  onMount(() => {
    if (cardElement) {
      const boundingClientRect = cardElement.getBoundingClientRect();

      cardData = {
        cardId: nanoid(),
        top: boundingClientRect.top + 12 * 16,
        left: boundingClientRect.left + 8 * 16,
      };
    }
  });

  $: if (
    $trackingCanvasState?.cardId &&
    $trackingCanvasState?.cardId !== cardData?.cardId
  ) {
    isOpen = false;
  }
</script>

<div class={clsx(containerClass, 'card-container', isOpen ? 'open' : '')}>
  <div bind:this={cardElement} class={clsx('card', isOpen ? 'open' : '')}>
    <div class={clsx('button-container', isOpen ? 'open' : '')}>
      <RoundButton on:click={toggleIsOpen} size="lg">
        <PlayIcon />
      </RoundButton>
    </div>
    <div class="card-header">
      <div class="flex items-center p-2 h-full">Task header</div>
    </div>
    <div class="p-4">Here be task description</div>
  </div>
</div>

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
      height: 24rem;
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
    @apply absolute top-0 left-0 button-container;
    z-index: 2;
  }
</style>
