import { createApp, provide, h } from "vue";
import { DefaultApolloClient } from "@vue/apollo-composable";
import { apolloClient } from "./apollo-client";
import App from "./App.vue";
import router from "./router";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

import "bootstrap/dist/css/bootstrap.css";

const app = createApp({
  setup() {
    provide(DefaultApolloClient, apolloClient);
  },
  render: () => h(App),
});

const vuetify = createVuetify({
  components,
  directives,
});

app.use(vuetify);
app.use(router).mount("#app");
