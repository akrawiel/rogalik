import { createEvent, createStore } from 'effector';

export interface OpenCardData {
  cardId: string;
  top: number;
  left: number;
}

export const onCardOpen = createEvent<OpenCardData>();
export const onCardClose = createEvent();

function createTrackingCanvas() {
  const store = createStore<OpenCardData | null>(null);

  store.on(onCardOpen, (_, payload) => payload);
  store.on(onCardClose, () => null);

  return store;
}

export default createTrackingCanvas();
