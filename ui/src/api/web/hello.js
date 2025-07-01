import request from '../request';

export function test1() {
  return request({
    url: '/hello/test1',
    method: 'get'
  });
}

export function test2() {
  return request({
    url: '/hello/test2',
    method: 'get'
  });
}