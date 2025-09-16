import { createRouter, createWebHistory } from "vue-router";
import Home from "@/pages/Home.vue";
import TunnelCreate from "@/pages/TunnelCreate.vue";
import TunnelList from "@/pages/TunnelList.vue";
import OpenSession from "@/pages/OpenSession.vue";

export default createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/tunnel/create",
      name: "tunnel-create",
      component: TunnelCreate,
    },
    {
      path: "/tunnel/list",
      name: "tunnel-list",
      component: TunnelList,
    },
    {
      path: "/open-session",
      name: "open-session",
      component: OpenSession,
    },
    {
      path: "/",
      name: "home",
      component: Home,
    },
  ],
});
