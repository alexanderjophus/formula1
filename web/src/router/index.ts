import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
import HomeView from "../views/HomeView.vue";

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "home",
    component: HomeView,
  },
  {
    path: "/drivers",
    name: "drivers",
    component: () => import("../views/DriversView.vue"),
  },
  {
    path: "/constructors",
    name: "constructors",
    component: () => import("../views/ConstructorsView.vue"),
  },
  {
    path: "/races",
    name: "races",
    component: () => import("../views/RacesView.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
