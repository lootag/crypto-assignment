Feature: Kraken Service

  Scenario: If I request the server time then the api returns it correctly
    When I request the server time
    Then the api successfully returns a valid server time

  Scenario: If I request the XBTUSD trading pair then the api returns it correctly
    When I request the XBTUSD trading pair
    Then the api successfully returns a valid XBTUSD trading pair

  Scenario: If I request my open orders then the api returns them correctly when I provide valid credentials
    Given that I provide a valid set of credentials 
    When I request my open orders
    Then the api successfully returns a valid set of open orders

  Scenario: If I request my open orders then the api will return an error if I provide the wrong credentials
    Given that I provide an invalid set of credentials 
    When I request my open orders
    Then the api does not return my open orders
