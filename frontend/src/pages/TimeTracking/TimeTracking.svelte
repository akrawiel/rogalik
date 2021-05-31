<script lang="ts">
  import clsx from 'clsx';

  import RoundButton from '@/atoms/RoundButton.svelte';
  import TaskCard from '@/organisms/TaskCard.svelte';
  import PlayIcon from '@/icons/PlayIcon.svelte';
  import trackingCanvasState from '@/store/trackingCanvas';

  import styles from './TimeTracking.module.scss';

  let mainCanvas: HTMLDivElement | undefined;
  let innerCanvas: HTMLDivElement | undefined;

  function isValidTaskButtonClass(
    taskButtonClass: string
  ): taskButtonClass is keyof typeof styles {
    return taskButtonClass in styles;
  }

  function getTaskButtonClassForIndex(taskIndex: number): string {
    const taskButtonClass = `task${taskIndex + 1}`;

    return isValidTaskButtonClass(taskButtonClass)
      ? styles[taskButtonClass]
      : '';
  }

  const tasks = [1, 2, 3, 4, 5];

  $: if (mainCanvas) {
    const mainCanvasBoundingRect = mainCanvas.getBoundingClientRect();

    const leftPlacement = $trackingCanvasState
      ? -$trackingCanvasState.left + mainCanvasBoundingRect.width / 2
      : 0;
    const topPlacement = $trackingCanvasState
      ? -$trackingCanvasState.top + mainCanvasBoundingRect.height / 2
      : 0;

    mainCanvas.style.left = `${leftPlacement}px`;
    mainCanvas.style.top = `${topPlacement}px`;
  }

  $: if (innerCanvas) {
    innerCanvas.style.transform = $trackingCanvasState
      ? 'translate(-50%, -50%) scale(1)'
      : 'translate(-50%, -50%) scale(0.75)';
  }
</script>

<div class="relative h-full overflow-hidden">
  <a href="/sign-out">Sign out</a>
  <div class="absolute w-full h-full main-canvas" bind:this={mainCanvas}>
    <div
      class={clsx(styles.buttonContainer, 'inner-canvas')}
      bind:this={innerCanvas}
    >
      <RoundButton size="xxl">
        <PlayIcon />
      </RoundButton>
      <div class={styles.tasksContainer}>
        {#each tasks as task, taskIndex}
          <TaskCard
            containerClass={clsx(
              getTaskButtonClassForIndex(taskIndex),
              styles.taskButtonContainer
            )}
          />
        {/each}
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  .main-canvas {
    @apply absolute;
    transition: all 0.25s ease-out;
  }

  .inner-canvas {
    transform: translate(-50%, -50%) scale(0.75);
    transition: all 0.25s ease-out;
  }
</style>
