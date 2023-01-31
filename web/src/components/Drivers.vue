<template>
  Year:
  <input v-model.lazy="variables.year" placeholder="current" />
  <div id="drivers">
    <p v-if="error">Something went wrong...</p>
    <p v-if="loading">Loading...</p>
    <div
      v-else
      v-for="driver in result.DriverStandings.drivers"
      :key="driver.Driver.code"
      class="driver-entry"
    >
      <b>{{ driver.Driver.code }}</b>
      {{ driver.points }}
      <a v-bind:href="driver.Driver.url">
        {{ driver.Driver.familyName.toUpperCase() }},
        {{ driver.Driver.givenName }}
      </a>
    </div>
  </div>
</template>

<script lang="ts">
import gql from "graphql-tag";
import { useQuery } from "@vue/apollo-composable";

const DRIVERS_QUERY = gql`
  query Drivers($year: String!) {
    DriverStandings(filter: { year: $year }) {
      drivers {
        points
        Driver {
          code
          givenName
          familyName
          url
        }
      }
    }
  }
`;

export default {
  name: "Drivers-Component",
  setup() {
    const initVariables = { year: "current" };
    const { result, loading, error, refetch, variables } = useQuery(
      DRIVERS_QUERY,
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
