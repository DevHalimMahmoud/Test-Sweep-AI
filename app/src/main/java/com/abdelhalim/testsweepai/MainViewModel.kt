package com.abdelhalim.testsweepai

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import com.abdelhalim.testsweepai.network.NetworkService
import dagger.hilt.android.lifecycle.HiltViewModel
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response
import javax.inject.Inject

@HiltViewModel
class MainViewModel @Inject constructor(private val networkService: NetworkService) : ViewModel() {

    private val _networkResponse = MutableLiveData<String>()
    val networkResponse: LiveData<String> get() = _networkResponse

    init {
        networkService.getExample().enqueue(object : Callback<String> {
            override fun onResponse(call: Call<String>, response: Response<String>) {
                _networkResponse.value = response.body()
            }

            override fun onFailure(call: Call<String>, t: Throwable) {
                _networkResponse.value = "Error: ${t.message}"
            }
        })
    }
}