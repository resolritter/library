export const devAPIURL = "localhost:8080"
export const apiURL = process.env.API_URL ?? devAPIURL

export const apiEndpoints = {
  createUser: function () {
    return "/user"
  },
}

export const routes = {
  login: function () {
    return "/login"
  },
  createUser: function () {
    return "/create_user"
  },
}
