package com.abdelhalim.testsweepai.network

import retrofit2.Call
import retrofit2.http.GET

interface NetworkService {
    @GET("example")
    fun getExample(): Call<String>
}