import type { Load } from "@sveltejs/kit"

export const load: Load = async ({fetch}) => {
  const data = await fetch("https://bonsai-health.shuttleapp.rs/user", {
    headers: {
      Authorization: "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJuYW1lIjoidGVzdCIsImV4cCI6MTY2NzY1MTExMH0.hMujQt5l_eoKOOi07ZlyBYe1mTQsmrCUbP2USBB3xds"
    }
  })

  console.log(await data.text())

  return {
    tree: []
  }
}