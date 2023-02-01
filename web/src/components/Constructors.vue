<template>
  Year:
  <input v-model.lazy="variables.year" placeholder="current" />
  <div id="constructors">
    <p v-if="error">Something went wrong...</p>
    <p v-if="loading">Loading...</p>
    <v-table v-else>
      <thead>
        <tr>
          <th>Position</th>
          <th>Team</th>
          <th>Points</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(team, index) in result.ConstructorStandings.teams"
          :key="index"
        >
          <td>{{ index + 1 }}</td>
          <td>
            <a :href="team.team.url" target="_blank">
              {{ team.team.name }}
            </a>
          </td>
          <td>{{ team.points }}</td>
        </tr>
      </tbody>
    </v-table>
  </div>
</template>

<style scoped>
#constructors {
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

const CONSTRUCTORS_QUERY = gql`
  query Constructors($year: String!) {
    ConstructorStandings(filter: { year: $year }) {
      teams {
        points
        team {
          id
          name
          url
        }
      }
    }
  }
`;

export default {
  name: "Constructors-Component",
  setup() {
    const initVariables = { year: "current" };
    const { result, loading, error, refetch, variables } = useQuery(
      CONSTRUCTORS_QUERY,
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
