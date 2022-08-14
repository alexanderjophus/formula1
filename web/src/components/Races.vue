<template>
  Year:
  <input v-model.lazy="variables.year" placeholder="current" />
  <div id="races">
    <p v-if="error">Something went wrong...</p>
    <p v-if="loading">Loading...</p>
    <p v-else v-for="race in result.Schedule.races" :key="race.round">
      <!-- {{ team.points }} <a v-bind:href="team.team.url">{{ team.team.name }}</a> -->
      {{ race.round }}
      <a v-bind:href="race.url">
        {{ race.raceName }}
      </a>
      @ {{ race.circuit.circuitName }}
    </p>
  </div>
</template>

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
