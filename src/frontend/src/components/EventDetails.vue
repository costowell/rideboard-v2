<script lang="ts" setup>
import CarTable from './CarTable.vue';
import EditEventButton from './EditEventButton.vue';
import IconPin from './icons/IconPin.vue';
import AddCarButton from './AddCarButton.vue';
import EditCarButton from './EditCarButton.vue';
</script>

<template>
  <div class="card">
    <div class="card-body">
      <div>
        <h3 class="card-title">{{ event?.name }}</h3>
        <h5 class="card-text">
          <a target="_blank" v-bind:href="'https://maps.google.com/?q=' + event?.location"
            ><IconPin /> {{ event?.location }}</a
          >
        </h5>
        <div v-if="screenStore.mobile">
          <h5><b>Start: </b>{{ startTime }}</h5>
          <h5><b>End: </b>{{ endTime }}</h5>
        </div>
        <h5 class="card-text" v-else>{{ startTime }} - {{ endTime }}</h5>
      </div>
      <div class="mt-4">
        <div class="d-flex justify-content-between align-items-center">
          <h4 class="mr-1">Cars</h4>
          <div v-if="!historyMode">
            <AddCarButton v-if="userCar == null" />
            <EditCarButton v-else :car="userCar" />
          </div>
        </div>
        <CarTable :eventId="event?.id" :key="event?.id" />
      </div>
      <EditEventButton v-if="userOwnsEvent && !historyMode" />
      <div>
        <i>Created by {{ event?.creator.name }}</i>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, inject, type PropType } from 'vue';
import { type Event } from '@/models';
import { format } from 'date-fns';
import { useAuthStore } from '@/stores/auth';
import { useScreenStore } from '@/stores/screen';
import { useEventStore } from '@/stores/events';

export default defineComponent({
  props: {
    event: Object as PropType<Event>
  },
  data() {
    let screenStore = useScreenStore();
    return {
      historyMode: inject('historyMode'),
      screenStore
    };
  },
  computed: {
    startTime() {
      let data = this.event!.startTime.toLocaleString();
      return format(data, this.screenStore.mobile ? 'MM/dd/yy hh:mm a' : 'LLLL dd, yyyy hh:mm a');
    },
    endTime() {
      let data = this.event!.endTime.toLocaleString();
      return format(data, this.screenStore.mobile ? 'MM/dd/yy hh:mm a' : 'LLLL dd, yyyy hh:mm a');
    },
    userOwnsEvent() {
      const authStore = useAuthStore();
      return this.event!.creator.id === authStore.user!.id;
    },
    userCar() {
      const eventStore = useEventStore();
      const authStore = useAuthStore();
      return eventStore.selectedEvent?.cars
        ?.filter((car) => car.driver.id === authStore.user?.id)
        .pop();
    }
  }
});
</script>
