export function findChangedFields<T>(oldObj: T, newObj: T): (keyof T)[] {
  // @ts-ignore
  return (Object.keys(oldObj) as (keyof T)[]).filter(key => oldObj[key] !== newObj[key]);
}

export function getChangedFields<T>(oldObj: T, newObj: T): Record<string, any> {
 const changed = findChangedFields(oldObj, newObj);
  let data: Record<string, any> = {};
  changed.forEach(key => {
    // @ts-ignore
    data[key] = newObj[key];
  });

  return data;
}