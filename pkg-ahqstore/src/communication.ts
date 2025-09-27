/**
 * This defines the types of `Events` that can be passed around
 * 
 * Events starting with `Request` indicate events 
 * that are sent to the daemon
 * 
 * These are internally wrapped for your convienience
 * 
 * @experimental
 */
export enum EventName {
  Search,
  AppFetch,
  AppAssetFetch,

  OnThemeUpdate,
  CommonStateUpdated,

  RequestInitialization,
  RequestState,
  RequestInjectCSS,
  RequestThemeData,
  RequestRestart,
  RequestFetch
}

/**
 * Determines the `EventType` received by the application
 * 
 * These are also internally managed by our plugin
 * @experimental
 */
export enum EventType {
  Request,
  Event,
  Response
}

/**
 * Response status provided by the Server
 * 
 * These are also internally managed
 * 
 * @experimental
 */
export enum ResponseStatus {
  Unauthorized,
  ServerError,
  /**
   * Request to terminate the plugin
   */
  Error_Terminate,
  Ok
}

/**
 * This whole interface is also internally managed by our plugin
 * @experimental
 */
export type CommunicationInterface =
  /**
   * This is the communication interface provided
   * if the event is a request
   * 
   * This gives a refId to use to fulfil the request from the
   * Plugin End.
   */
  {
    /**
     * Event Name {@link EventName}
     */
    event: EventName;
    /**
     * Event Type {@link EventType}
     * 
     * Set to {@link EventType.Request}
     */
    eventType: EventType.Request;
    /**
     * A reference id
     */
    refId: number;
    /**
     * Data to be attested
     */
    data: unknown;
  } |
  {
    /**
     * Event Name {@link EventName}
     */
    event: EventName;
    /**
     * Event Type {@link EventType}
     * 
     * Set to {@link EventType.Request}
     */
    eventType: EventType.Event;
    /**
     * Data to be attested
     */
    data: unknown;
  } |
  /**
   * This is the communication interface provided
   * if the event is a response
   * 
   * This gives an eventType and finally response data
   */
  {
    /**
      * Event Type {@link EventType}
      * 
      * Set to {@link EventType.Response}
    */
    eventType: EventType.Response;
    status: ResponseStatus;
    /**
     * A reference id
     */
    refId: number;
    /**
     * Data whose type is not yet known
     */
    data: unknown
  };