package com.squireofsoftware.cashier.acknowledgements

import com.squireofsoftware.cashier.order.OrderService
import io.mockk.MockKAnnotations
import io.mockk.impl.annotations.MockK
import okhttp3.mockwebserver.MockWebServer
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource

class AcknowledgementControllerTest {
    @MockK
    lateinit var orderServiceMock: OrderService

    private val proxyServer = MockWebServer()

    @BeforeEach
    fun setup() {
        MockKAnnotations.init(this, relaxUnitFun = true)
        proxyServer.start()
    }

    @AfterEach
    fun cleanUp() {
        proxyServer.shutdown()
    }

    @ParameterizedTest
    @ValueSource(booleans = [true, false])
    fun test(isProxied: Boolean) {
        val testUrl = "http://hello.world/"
        val controller = AcknowledgementController(orderServiceMock, isProxied)

        try{
            controller.testProxy(proxyServer.hostName, proxyServer.port, testUrl)
        } catch (e: Exception) {
            if (isProxied) {
                assert(proxyServer.requestCount == 1)
            } else {
                assert(proxyServer.requestCount == 0)
            }
        }
    }
}