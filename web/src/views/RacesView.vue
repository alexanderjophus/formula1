<template>
  <div class="about">
    <h1>Schedule</h1>
  </div>
  <Races />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import Races from "@/components/Races.vue";

export default defineComponent({
  name: "RacesView",
  components: {
    Races,
  },
});
</script>
