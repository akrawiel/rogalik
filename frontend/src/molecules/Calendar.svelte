<script>
  import dayjs from "dayjs";

  const today = dayjs();

  let currentDayObject = today.clone();

  $: startOfCurrentMonth = currentDayObject.startOf("month").startOf("week");
  $: endOfCurrentMonth = currentDayObject.endOf("month").endOf("week");

  $: currentMonthDays = endOfCurrentMonth.diff(startOfCurrentMonth, "day") + 1;

  $: shownDays = [...Array(currentMonthDays).keys()].map((dayIndex) => {
    const currentDay = startOfCurrentMonth.add(dayIndex, "day");

    return {
      key: dayIndex,
      label: currentDay.format("D"),
      active: currentDay.month() === currentDayObject.month(),
    };
  });

  $: weekdayLabels = [...Array(7).keys()].map((dayIndex) =>
    startOfCurrentMonth.add(dayIndex, "day").format("dd")
  );

  $: currentMonthLabel = currentDayObject.format("MMMM YYYY");

  const previousMonth = () => {
    currentDayObject = currentDayObject.subtract(1, "month");
  };

  const nextMonth = () => {
    currentDayObject = currentDayObject.add(1, "month");
  };
</script>

<style lang="scss">
  .m-calendar {
    background-color: var(--secondary);
    border-radius: 8px;
    color: var(--white);
    max-width: 320px;
    padding: var(--spacing1);

    &__topRow {
      align-items: center;
      display: flex;
      justify-content: space-evenly;
      margin-bottom: var(--spacing2);
    }

    &__changeMonthButton {
      cursor: pointer;
      font-size: var(--fontSize4);
      height: var(--fontSize4);
    }

    &__monthLabel {
      user-select: none;
    }

    &__daysContainer {
      display: grid;
      justify-items: center;
      grid-template-columns: repeat(7, 1fr);
    }

    &__weekday {
      color: var(--white);
      font-size: var(--fontSize2);
      margin: var(--spacing1);
      opacity: 0.6;
      user-select: none;
    }

    &__day {
      align-items: center;
      color: var(--grey);
      border-radius: 24px;
      cursor: pointer;
      display: flex;
      font-size: var(--fontSize3);
      height: 24px;
      margin: var(--spacing1);
      justify-content: center;
      user-select: none;
      width: 24px;

      &.-active {
        color: var(--white);
      }
    }
  }
</style>

<div class="m-calendar" on:mousedown|preventDefault>
  <div class="m-calendar__topRow">
    <div
      class="m-calendar__changeMonthButton"
      on:click|preventDefault|stopPropagation={previousMonth}>
      <i class="mi-arrow-left" />
    </div>
    <div class="m-calendar__monthLabel">{currentMonthLabel}</div>
    <div
      class="m-calendar__changeMonthButton"
      on:click|preventDefault|stopPropagation={nextMonth}>
      <i class="mi-arrow-right" />
    </div>
  </div>
  <div class="m-calendar__daysContainer">
    {#each weekdayLabels as weekday}
      <div class="m-calendar__weekday">{weekday}</div>
    {/each}
    {#each shownDays as day (day.key)}
      <div class="m-calendar__day" class:-active={day.active}>{day.label}</div>
    {/each}
  </div>
</div>
