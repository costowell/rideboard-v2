<script setup lang="ts">
import JoinCarButton from './JoinCarButton.vue';
import LeaveCarButton from './LeaveCarButton.vue';
</script>

<template>
  <div class="m-1">
    <h5>Driver Comment:</h5>
    <p class="car-comment" v-if="car!.comment.length != 0">{{ car!.comment }}</p>
    <p v-else><i>No Comment Provided</i></p>
    <h5>Passengers:</h5>
    <ul class="no-bullets" v-if="car!.riders.length != 0">
      <li v-for="(rider, index) in car!.riders" :key="index">
        {{ rider.name }} ({{ rider.email }})
      </li>
    </ul>
    <p v-else><i>No Riders</i></p>
    <LeaveCarButton v-if="userInCar" :carId="car!.id" />
    <JoinCarButton
      v-else-if="car!.riders.length < car!.maxCapacity && userCanJoinCar"
      :carId="car?.id"
    />
  </div>
</template>

<script lang="ts">
import { type Car } from '@/models';
import { defineComponent, type PropType } from 'vue';
import { useAuthStore } from '@/stores/auth';
import { useEventStore } from '@/stores/events';

export default defineComponent({
  props: {
    car: Object as PropType<Car>,
    eventId: Number
  },
  computed: {
    userCanJoinCar() {
      const authStore = useAuthStore();
      const eventStore = useEventStore();
      return !(
        eventStore.selectedEvent?.cars?.map((car) => car.driver.id).includes(authStore.user!.id) ||
        eventStore.selectedEvent?.cars
          ?.map((car) => car.riders)
          .flat()
          .map((rider) => rider.id)
          .includes(authStore.user!.id)
      );
    },
    userInCar() {
      const authStore = useAuthStore();
      return this.car?.riders.find((rider) => rider.id === authStore.user!.id);
    }
  }
});
</script>

<style>
ul.no-bullets {
  list-style-type: none; /* Remove bullets */
  padding: 0; /* Remove padding */
  margin: 0; /* Remove margins */
}
.car-comment {
  white-space: normal;
}
</style>
