import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import CreateTunnel from "@/components/CreateTunnel.vue";

export default createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/create-tunnel",
      name: "create-tunnel",
      component: CreateTunnel,
    },
    {
      path: "/",
      name: "home",
      component: Home,
    },
  ],
});
