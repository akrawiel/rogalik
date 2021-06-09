<script lang="ts">
  import clsx from 'clsx';

  import RoundButton from '@/atoms/RoundButton.svelte';
  import TaskCard from '@/organisms/TaskCard.svelte';
  import PlayIcon from '@/icons/PlayIcon.svelte';
  import trackingCanvasState from '@/store/trackingCanvas';

  import styles from './TimeTracking.module.scss';

  let innerCanvas: HTMLDivElement | undefined;
  let mainCanvas: HTMLDivElement | undefined;

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

  const tasks = [
    {
      id: '60dc9cad-d4cb-4d09-a6fa-f6c0ce793942',
      name: 'vel',
      description:
        'molestie hendrerit at vulputate vitae nisl aenean lectus pellentesque eget nunc donec quis orci eget',
      assigneeId: '2b2cb860-aab5-48f3-b73f-8a0c48f865d1',
    },
    {
      id: '6fbe8929-24e3-46fe-be2d-860fe90f5670',
      name: 'tristique',
    },
    {
      id: '8dcf64e9-98a1-4188-a75b-83f27cf95ddb',
      name: 'nunc viverra dapibus',
      description:
        'interdum venenatis turpis enim blandit mi in porttitor pede justo eu massa donec dapibus duis at velit eu',
      assigneeId: 'ebab3e42-9ba1-4418-a67a-5fd242f38550',
    },
    {
      id: '36dded23-1a91-4e2c-bc72-b5f906c766fe',
      name: 'magna',
      description:
        'sem sed sagittis nam congue risus semper porta volutpat quam pede lobortis ligula sit amet',
      assigneeId: '47a5be7e-d011-434e-99a9-78c20fc24694',
    },
    {
      id: 'c600f382-f55a-4efd-91cf-0868ce136318',
      name: 'non sodales',
    },
  ];

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
    innerCanvas.style.opacity = '1';
    innerCanvas.style.transform = $trackingCanvasState
      ? 'translate(-50%, -50%) scale(1)'
      : 'translate(-50%, -50%) scale(0.75)';
  }
</script>

<div class="relative w-full h-full overflow-hidden">
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
            {task}
          />
        {/each}
      </div>
    </div>
  </div>
  <div class="absolute w-full top-0 left-0">
    <a href="/sign-out">Sign out</a>
  </div>
</div>

<style lang="scss">
  .main-canvas {
    @apply absolute;
    transition: all 0.25s ease-out;
  }

  .inner-canvas {
    opacity: 0;
    transform: translate(-50%, -50%) scale(1);
    transition: all 0.25s ease-out;
  }
</style>
