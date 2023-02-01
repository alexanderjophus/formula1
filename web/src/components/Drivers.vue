<template>
  Year:
  <input v-model.lazy="variables.year" placeholder="current" />
  <div id="drivers">
    <p v-if="error">Something went wrong...</p>
    <p v-if="loading">Loading...</p>
    <v-table v-else>
      <thead>
        <tr>
          <th>Position</th>
          <th>Code</th>
          <th>Driver</th>
          <th>Points</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(driver, index) in result.DriverStandings.drivers"
          :key="index"
        >
          <td>{{ index + 1 }}</td>
          <td>
            <b>
              {{ driver.Driver.code }}
            </b>
          </td>
          <td>
            <a :href="driver.Driver.url" target="_blank">
              {{ driver.Driver.givenName }} {{ driver.Driver.familyName }}
            </a>
          </td>
          <td>{{ driver.points }}</td>
        </tr>
      </tbody>
    </v-table>
  </div>
</template>

<style scoped>
#drivers {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>

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
