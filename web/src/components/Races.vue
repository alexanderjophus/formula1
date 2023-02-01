<template>
  Year:
  <input v-model.lazy="variables.year" placeholder="current" />
  <div id="races">
    <p v-if="error">Something went wrong...</p>
    <p v-if="loading">Loading...</p>
    <v-table v-else>
      <thead>
        <tr>
          <th>Round</th>
          <th>Name</th>
          <th>Date</th>
          <th>Circuit</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(race, index) in result.Schedule.races" :key="index">
          <td>{{ race.round }}</td>
          <td>
            <a :href="race.url" target="_blank">
              {{ race.raceName }}
            </a>
          </td>
          <td>{{ race.date }}@{{ race.time }}</td>
          <td>
            <div v-if="race.circuit.img" class="hover_img">
              <a href="#">
                {{ race.circuit.circuitName }}
                <span>
                  <img :src="race.circuit.img" alt="image" height="100" />
                </span>
              </a>
            </div>
            <div v-else>
              {{ race.circuit.circuitName }}
            </div>
          </td>
        </tr>
      </tbody>
    </v-table>
  </div>
</template>

<style scoped>
#races {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.hover_img a {
  position: relative;
}

.hover_img a span {
  position: absolute;
  display: none;
  z-index: 99;
}

.hover_img a:hover span {
  display: block;
  background-color: White;
}
</style>

<script lang="ts">
import gql from "graphql-tag";
import { useQuery } from "@vue/apollo-composable";

const RACES_QUERY = gql`
  query Races($year: String!) {
    Schedule(year: $year) {
      season
      races {
        round
        url
        raceName
        date
        time
        circuit {
          img
          circuitName
        }
      }
    }
  }
`;

export default {
  name: "Races-Component",
  setup() {
    const initVariables = { year: "current" };
    const { result, loading, error, refetch, variables } = useQuery(
      RACES_QUERY,
      initVariables
    );
    return {
      result,
      loading,
      error,
      refetch,
      variables,
    };
  },
};
</script>
