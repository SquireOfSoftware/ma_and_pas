<script>
import { useQuery } from '@vue/apollo-composable'
import gql from 'graphql-tag'
// follow these instructions https://www.apollographql.com/blog/frontend/getting-started-with-vue-apollo/#initial-vue-apollo-config
export default {
  setup () {
    const { result, loading } = useQuery(gql`
      query testquery {
          burgers {
            id
            name
            active
            cost
          }
          drinks {
            id
            name
            cost
          }
          sides {
            id
            name
          }
        }
    `)

    console.log({result})

    return {
      result,
      loading,
    }
  },
}
</script>

<template>
  <div v-if="loading">Loading...</div>
  <ul v-else-if="result && (result.burgers || result.drinks || result.sides)">
    <li v-for="burger of result.burgers" :key="burger.id">
      {{ burger.id }}
    </li>
    <li v-for="side of result.sides" :key="side.id">
      {{ side.id }}
    </li>
    <li v-for="drink of result.drinks" :key="drink.id">
      {{ drink.id }}
    </li>
  </ul>
</template>